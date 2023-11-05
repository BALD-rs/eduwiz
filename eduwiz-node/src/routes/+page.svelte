<script>
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { logIn, user } from '$lib/flow/utils';

	onMount(() => {
		if ($user.loggedIn) {
			goto('/home');
		}
	});
</script>

<svelte:head>
	<title>Login</title>
	<meta name="description" content="Login page" />
</svelte:head>

<div class="top-bar">
	<h1 style="color: white;">EduWiz</h1>
	<button on:click={() => goto('/about')}>About</button>
</div>

<div class="login-container">
	<div class="login-box">
		<img src="logo-words.png" alt="logo" />
		<h2>Please log in to continue</h2>
		<div class="buttons">
			<button on:click={logIn}>Log In</button>
		</div>
	</div>
</div>

<style>
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

	div.top-bar h1 {
		margin: 0 20px;
	}

</style>
