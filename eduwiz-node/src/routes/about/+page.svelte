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
	<title>About</title>
	<meta name="description" content="About page" />
</svelte:head>

<div class="top-bar">
	<h1 style="color: white;">EduWiz</h1>
	<button on:click={() => goto('/')}>Home</button>
</div>
<div class="main">
	<div class="center-box" style="margin: 10px auto"> 
		<h1><b>A little bit about us...</b></h1>
		<h2 style="text-align: justify; margin: 20px">
			&emsp;&emsp; EduWiz was developed as a submission for Hack K-State 2023 
			with the goal of creating a decentralized education. The project 
			was written mostly using Rust, Svelte, and TypeScript. Although the
			codebase was hastily developed, we had an incredible time 
			designing and implementing EduWiz, and the knowledge we gained
			will help us all prepare for the world ahead. Thank you all for 
			experiencing our project!</h2>
			<h2>-The EduWiz Team</h2>
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
	
	div.center-box {
		width: 800px;
		padding: 2em;
		background: white;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		border-radius: 15px;
		text-align: center;
		box-sizing: border-box;
	}
</style>
