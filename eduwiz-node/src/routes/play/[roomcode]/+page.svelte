<script lang="ts">
	import { quizComplete } from '$lib/flow/utils.js';
	import { onMount } from 'svelte';
	import { writable } from 'svelte/store';
	import { goto } from '$app/navigation';

	export let data;
	$: question = '';
	$: choices = ['', '', '', ''];
	let status = 'WAITING TO START';
	let numCorrect = 0;
	let numAnswered = 0;

	const loading = writable(false);

	const submitAnswer = async (answer: string) => {
		const previousStatus = status; // to check later on whether this is the first empty submission
		status = '';
		const reqBody = {
			user: data.username,
			room: data.roomCode,
			question: question,
			answer: answer
		};
		console.log(reqBody);
		const res = await fetch(import.meta.env.VITE_URL + 'submit_answer', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(reqBody)
		});
		const resJson = await res.json();
		console.log(resJson);
		question = resJson.prompt;
		choices = resJson.answers;
		if (previousStatus == 'WAITING TO START') {
			status = 'SHOW QUESTION';
		} else {
			numAnswered++;
			numCorrect = resJson.last_correct ? numCorrect + 1 : numCorrect;
			status = resJson.last_correct ? 'CORRECT' : 'INCORRECT';
			setTimeout(() => {
				if (status != 'GAME OVER') status = 'SHOW QUESTION';
			}, 3000);
		}
	};

	const handleChoiceClick = (answer: string) => {
		console.log(`Answer choice "${answer}" clicked`);
		submitAnswer(answer);
	};

	async function levelUp() {
		loading.set(true);
		await quizComplete();
		loading.set(false);
		goto('/home');
	}

	const colors = ['#0075ff', '#0075ff', '#0075ff', '#0075ff'];

	let socket;
	onMount(async () => {
		window.onbeforeunload = (e) => '';
		socket = new WebSocket(import.meta.env.VITE_WS + `join_room/${data.roomCode}/${data.username}`);
		socket.onopen = () => {
			console.log('WebSocket connection established');
		};

		socket.onmessage = async (event) => {
			console.log('Message from server:', event.data);
			switch (event.data) {
				case 'START':
					// submit an empty answer to receive the first question
					await submitAnswer(' ');
					break;
				case 'END':
					status = 'GAME OVER';
					break;
			}
		};

		socket.onclose = () => {
			console.log('WebSocket connection closed');
		};

		socket.onerror = (error) => {
			console.error('WebSocket error:', error);
		};
	});
</script>

<svelte:head>
	<title>Room {data.roomCode}</title>
	<meta name="description" content="Game Room" />
</svelte:head>

{#if status == 'WAITING TO START'}
	<div class="login-container">
		<div class="username-panel">
			<h2>Waiting for the game to begin...</h2>
			<div class="spinner" />
		</div>
	</div>
{:else if status == 'SHOW QUESTION'}
	<div class="question">
		<h1>{question}</h1>
		<div class="choices">
			{#each choices as choice, i (i)}
				<button
					class="choice"
					style="background-color: {colors[i]};"
					on:click={() => {
						handleChoiceClick(choice);
					}}>{choice}</button
				>
			{/each}
		</div>
	</div>
{:else if status == 'CORRECT'}
	<div class="main">
		<div class="center-box">
			<h1>✅</h1>
			<p style="color: green;">CORRECT!</p>
			<p>{numCorrect}/{numAnswered}</p>
		</div>
	</div>
{:else if status == 'INCORRECT'}
	<div class="main">
		<div class="center-box">
			<h1>❌</h1>
			<p style="color: red;">INCORRECT!</p>
			<p>{numCorrect}/{numAnswered}</p>
		</div>
	</div>
{:else if status == 'GAME OVER'}
	<div class="login-container">
		<div class="username-panel">
			{#if $loading}
				<h2>Updating the blockchain...</h2>
				<div class="spinner" />
			{:else}
				<p>Game Over!</p>
				<p>{numCorrect}/{numAnswered} Correct</p>
				<button
					on:click={async () => {
						levelUp();
					}}>Level up!</button
				>
			{/if}
		</div>
	</div>
{/if}

<style>
	div.question h1 {
		font-size: 4em;
	}

	div.choices {
		font-size: 4em;
	}

	.choice {
		border-radius: 25px;
	}

	div.main,
	div.login-container {
		display: flex;
		justify-content: center;
		align-items: center;
		height: 90vh;
	}

	div.center-box,
	div.username-panel {
		width: 450px;
		padding: 2em;
		background: white;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		border-radius: 15px;
		text-align: center;
		box-sizing: border-box;
		margin: 30px;
		font-size: 1.5em;
	}

	div.center-box h1 {
		margin: 0.5em 0;
		font-size: 6em;
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
