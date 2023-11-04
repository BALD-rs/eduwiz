<script lang="ts">
	import { onMount } from 'svelte';
	import type { RoomCode } from './+page';
	// import Question from './Question.svelte';

	export let data: RoomCode;
	$: question = '';
	$: choices = ['', '', '', ''];
	let showQuestion = false;

	const submitAnswer = async (answer: string) => {
		const res = await fetch('http://localhost:3000/api/submit_answer', {
			method: 'POST',
			body: JSON.stringify({
				user: 'freddy fazbear',
				room: data.roomCode,
				question: question,
				answer: answer
			})
		});
		console.log(res.status, await res.json())
		showQuestion = false;
	}

	const handleChoiceClick = (answer: string) => {
		console.log(`Answer choice "${answer}" clicked`);
		submitAnswer(answer);
	};

	const colors = ['#ff2200', '#0fd637', '#0080ff', '#ff00ff'];

	let socket;
	onMount(async () => {
		// const res = await fetch(`http://localhost:3000/api/join_room/${data.roomCode}`);
		// console.log(res);
		// console.log(res.status)
		socket = new WebSocket(`ws://localhost:3000/api/join_room/${data.roomCode}`);
		socket.onopen = () => {
			console.log('WebSocket connection established');
			showQuestion = true;
			// submit an empty answer to receive the first question
			submitAnswer('');
		};

		socket.onmessage = (event) => {
			console.log('Message from server:', event.data);
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
			{#each choices as choice, i (choice)}
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
{:else}
	<p>waiting for a question...</p>
{/if}
