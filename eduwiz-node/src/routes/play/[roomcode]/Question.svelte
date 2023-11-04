<script lang="ts">
    export let question: string;
    export let choices: string[];

    const handleChoiceClick = (choice: string) => {
        console.log(`${choice} clicked`);
    }
</script>

<div class="question">
    <h1>{question}</h1>
    {#each choices as choice}
    <button on:click={() => {handleChoiceClick(choice)}}>{choice}</button>
    {/each}
</div>