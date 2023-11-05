<script lang="ts">
	import { quizComplete } from '$lib/flow/utils.js';
	import { onMount } from 'svelte';

	export let data;
	$: question = '';
	$: choices = ['', '', '', ''];
	let status = 'WAITING TO START';
	let numCorrect = 0;
	let numAnswered = 0;

	const submitAnswer = async (answer: string) => {
		const previousStatus = status; // to check later on whether this is the first empty submission
		status = '';
		const reqBody = {
			user: data.username,
			room: data.roomCode,
			question: question,
			answer: answer
		};
		console.log(reqBody)
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

	const colors = ['#ff2200', '#0fd637', '#0080ff', '#ff00ff'];

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
	<p>Waiting for teacher to start the game</p>
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
	<p style="color: green;">YOU GOT THE QUESTION RIGHT</p>
	<p>{numCorrect}/{numAnswered} so far</p>
{:else if status == 'INCORRECT'}
	<p style="color: red;">YOU GOT THE QUESTION WRONG</p>
	<p>{numCorrect}/{numAnswered} so far</p>
{:else if status == 'GAME OVER'}
	<p>game over! you got {numCorrect}/{numAnswered} right. <a href="/home">return to home</a></p>
	<button on:click={quizComplete}>level up!</button>
{/if}
