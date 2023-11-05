<script lang="ts">
	import { goto } from '$app/navigation';
	import { setUsername } from '$lib/flow/utils';
	import { writable } from 'svelte/store';

	const loading = writable(false);

	async function updateUsername() {
		const inputElement = document.querySelector('input#username') as HTMLInputElement;
		loading.set(true);
		await setUsername(inputElement?.value);
		loading.set(false);
		goto('/home');
	}
</script>

<svelte:head>
	<title>Onboarding</title>
	<meta name="description" content="Onboarding" />
</svelte:head>

<div class="login-container">
	<div class="username-panel">
		{#if $loading}
			<h2>Updating username...</h2>
			<div class="spinner" />
		{:else}
			<h2>Choose a username:</h2>
			<form class="internal-panel">
				<input type="text" name="username" id="username" />
				<button on:click={async () => updateUsername()}>Submit</button>
			</form>
		{/if}
	</div>
</div>

<style>
	#username {
		padding: 10px;
		margin: 10px;
		width: 100%;
	}

	.spinner {
		width: 40px;
		height: 40px;
		border: 4px solid #f3f3f3;
		border-top: 4px solid #3498db;
		border-radius: 50%;
		animation: spin 2s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
