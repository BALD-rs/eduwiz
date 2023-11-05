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

<div id="header-bar">
	<h1 style="color: white">EduWiz</h1>
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


