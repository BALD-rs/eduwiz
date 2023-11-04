<script>
	import { goto } from '$app/navigation';
	import { logIn, user } from '$lib/flow/utils';
</script>

<svelte:head>
	<title>Login</title>
	<meta name="description" content="Login page" />
</svelte:head>

{#if $user.loggedIn}
	{goto('/home')}
{/if}

<section>
	<img width="10%" src="logo-words.png" alt="logo" />
	<h2>Please log in to continue</h2>
	<div class="buttons">
		<button on:click={logIn}>Log In</button>
	</div>
	<!-- <h2>Welcome, {$user.addr}!</h2>
    <div class="buttons">
      <button on:click={logOut}>Log Out</button>
    </div> -->
</section>
