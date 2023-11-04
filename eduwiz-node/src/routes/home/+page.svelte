<script>
	import { goto } from '$app/navigation';
	import { getAccount, logOut } from '$lib/flow/utils';
	import { onMount } from 'svelte';

	let roomCode = '';
	$: tokens = '';

	function joinRoom() {
		goto(`/play/${roomCode}`);
	}

	onMount(async () => {
		const account = await getAccount();
		console.log(account);
		tokens = account.balance;
	});
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Homepage" />
</svelte:head>

<div class="topnav">
	<div class="input">
		<span>Flow tokens: {tokens}</span>
		<input type="text" placeholder="Enter room code" bind:value={roomCode} />
		<button on:click={joinRoom} class="button">Join Room</button>
		<button on:click={logOut} class="button">Logout</button>
	</div>
</div>
