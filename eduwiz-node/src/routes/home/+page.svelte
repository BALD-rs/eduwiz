<script lang='ts'>
	import { goto } from '$app/navigation';
	import { getAccount, getLevel, getQuizzesComplete, getUsername, logOut, setUsername } from '$lib/flow/utils';
	import { onMount } from 'svelte';

	let roomCode = '';
	let username = '';
	let quizzesCompleted: number;
	let level: number;

	function joinRoom() {
		goto(`/play/${roomCode}`);
	}

	onMount(async () => {
		const account = await getAccount();
		console.log(account);
		console.log(account.address);
		username = await getUsername();
		console.log(username);
		quizzesCompleted = await getQuizzesComplete();
		console.log(quizzesCompleted);
		level = await getLevel();
		console.log(level);
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Homepage" />
</svelte:head>

<div class="top-bar">
	<button on:click={() => goto('/onboard')}>Edit Username</button>
	<button on:click={logOut}>Log Out</button>
</div>
<div class="main">
	<div class="center-box">
		<img src={`/shields/shield-${level}.png`} alt="shield" style="width: 60%" />
		<h1><b>Welcome, {username}</b>!</h1>
		<h2><b>Level:</b> {level}</h2>
		<h2><b>Quizzes Completed:</b> {quizzesCompleted}</h2>
	</div>
	<div class="center-box">
		<img src="/logo-words.png" alt="logo" />
		<div class="input">
			<input type="text" placeholder="Enter room code" id="room-code" bind:value={roomCode} />
			<button on:click={joinRoom} class="button" id="join-room">Join Room</button>
		</div>
	</div>
	
</div>

<style>
	div.main {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 90vh;
	}

	div.top-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		padding: 0 1em;
		background-color: blue;
		color: white;
		height: 80px; 
	}

	div.top-bar button {
		margin: 0 20px;
		box-shadow: 0 0px 5px rgba(1, 1, 80, 255);
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
	}

	div.input {
		display: flex;
		flex-direction: column;
		align-items: center;
		padding: 0 15%;
	}

	#room-code {
		width: 95%;
		padding: 10px;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
		margin-bottom: 15px;
	}

	#join-room {
		width: 100%;
		padding: 10px;
	}

	.button {
		margin: 10px 10px;
	}
</style>
