<script lang="ts">
	import type { RoomCode } from './+page';
	import Question from './Question.svelte';

	export let data: RoomCode;
	$: question = 'this is a test question';
	$: choices = ['choice A', 'choice B', 'choice C', 'choice D'];
	let showQuestion = true;
</script>

<svelte:head>
	<title>Room {data.roomCode}</title>
	<meta name="description" content="Game Room" />
</svelte:head>

{#if showQuestion}
	<Question {question} {choices} />
{:else}
	<p>waiting for a question...</p>
{/if}
