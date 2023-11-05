<script lang="ts">
	import { onMount } from 'svelte';
	import type { RoomCode } from './+page';

	export let data: RoomCode;
	$: question = '';
	$: choices = ['', '', '', ''];
	let showQuestion = false;
	let gameOver = false;

	const submitAnswer = async (answer: string) => {
		showQuestion = false;
		const res = await fetch('http://127.0.0.1:3000/api/submit_answer', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify({
				user: 'freddy fazbear',
				room: data.roomCode,
				question: question,
				answer: answer
			})
		});
		const resJson = await res.json();
		console.log(resJson);
		question = resJson.prompt;
		choices = resJson.answers;
		showQuestion = true;
	};

	const handleChoiceClick = (answer: string) => {
		console.log(`Answer choice "${answer}" clicked`);
		submitAnswer(answer);
	};

	const colors = ['#ff2200', '#0fd637', '#0080ff', '#ff00ff'];

	let socket;
	onMount(async () => {
		socket = new WebSocket(`ws://127.0.0.1:3000/api/join_room/${data.roomCode}`);
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
					showQuestion = false;
					gameOver = true;
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

{#if showQuestion}
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
{:else if gameOver}
	<p>game over! go <a href="/home">home</a></p>
{:else}
	<p>waiting for a question...</p>
{/if}
