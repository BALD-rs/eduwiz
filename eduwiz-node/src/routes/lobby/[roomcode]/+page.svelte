<script lang="ts">
	import { onDestroy } from 'svelte';
	import type { RoomCode } from './+page';

	export let data: RoomCode;

	let socket: WebSocket;
	function start() {
		killInterval();
		socket = new WebSocket(import.meta.env.VITE_WS + `start_room/${data.roomCode}`);
		socket.onmessage = async (event) => {
			console.log('Message from server:', event.data);
			players = JSON.parse(event.data);
			const tbody = document.createElement('tbody');
			for (let i = 0; i < players.length; i++) {
				const tr = document.createElement('tr');
				for (let j = 0; j < 2; j++) {
					const td = document.createElement('td');
					td.innerText = players[i][j];
					tr.append(td);
				}
				tbody.append(tr);
			}
			const leaderboard = document.getElementById('leaderboard');
			if (leaderboard != null) {
				leaderboard.innerHTML = tbody.innerHTML;
			}
		};
	}

	async function getPlayers() {
		const res = await fetch(import.meta.env.VITE_URL + `get_users/${data.roomCode}`);
		const json = await res.json();
		const users: string[] = json.users;

		const newList = document.createElement('ul');
		users.forEach((user: string) => {
			const li = document.createElement('li');
			li.innerText = user;
			newList.append(li);
		});
		const oldList = document.getElementById('playerList');
		if (oldList != null) {
			oldList.innerHTML = newList.innerHTML;
		}
	}

	function killInterval() {
		clearInterval(interval);
	}

	const interval = setInterval(getPlayers, 1000);

	onDestroy(() => {
		killInterval();
	});

	let players: any[];
</script>

<svelte:head>
	<title>Lobby - {data.roomCode}</title>
	<meta name="description" content="Lobby" />
</svelte:head>

<div>
	{#if socket == null}
		<h1>Code</h1>
		<h2>{data.roomCode}</h2>
		<ul id="playerList" />
		<button on:click={start}>Start</button>
	{:else}
		<h1>Leaderboard</h1>
		<table>
			<thead>
				<tr>
					<th> Player </th>
					<th> Score </th>
				</tr>
			</thead>
			<tbody id="leaderboard" />
		</table>
	{/if}
</div>
