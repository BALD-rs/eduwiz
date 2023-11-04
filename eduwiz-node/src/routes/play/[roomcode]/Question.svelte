<script lang="ts">
	export let question: string;
	export let choices: string[];

	const handleChoiceClick = (choice: string) => {
		console.log(`${choice} clicked`);
	};

	const colors = ['red', 'orange', 'green', 'blue'];
</script>

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
