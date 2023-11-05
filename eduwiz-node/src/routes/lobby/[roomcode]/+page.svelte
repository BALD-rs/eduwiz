<script lang="ts">
	import { onDestroy, onMount } from 'svelte';
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
			const trSet = new Set();
			for (let i = 0; i < players.length; i++) {
				const tr = document.createElement('tr');
				for (let j = 0; j < 2; j++) {
					const td = document.createElement('td');
					td.innerText = players[i][j];
					tr.append(td);
				}
				const trString = tr.innerHTML;
				if (!trSet.has(trString)) {
					tbody.append(tr);
					trSet.add(trString);
				}
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

	onMount(() => {
		window.onbeforeunload = (e) => '';
	});

	onDestroy(() => {
		killInterval();
	});

	let players: any[];
</script>

<svelte:head>
	<title>Lobby - {data.roomCode}</title>
	<meta name="description" content="Lobby" />
</svelte:head>

<div class="main">
	{#if socket == null}
		<div class="center-box">
			<h1>Code</h1>
			<h2>{data.roomCode}</h2>
			<ul id="playerList" />
			<button on:click={start}>Start</button>
		</div>
	{:else}
		<div class="center-box flex">
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
		</div>
	{/if}
</div>

<style>
	div.main {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 90vh;
	}

	div.center-box {
		width: 450px;
		padding: 2em;
		background: white;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		border-radius: 15px;
		text-align: center;
		box-sizing: border-box;
		margin: 30px;
		font-size: 2em;
	}
	th {
		padding-left: 20px;
		padding-right: 20px;
	}

	ul {
		list-style-type: none;
		margin: 0;
		margin-bottom: 50px;
		padding: 0;
	}

	.flex {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}
</style>
