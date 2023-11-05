<script lang="ts">
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	let questions: any[] = [];
	let timeLimit: number = 60;

	onMount(() => {
		const storedQuestions = localStorage.getItem('questions');
		if (storedQuestions) {
			questions = JSON.parse(storedQuestions);
		}
	});

	const addQuestion = () => {
		questions.push({
			prompt: '',
			answers: ['', '', '', ''],
			correct_answer: ''
		});
		questions = questions;
	};

	const removeQuestion = () => {
		questions.pop();
		questions = questions;
	};

	const createGame = async () => {
		localStorage.setItem('questions', JSON.stringify(questions));
		questions.forEach((question) => {
			question.correct_answer = question.answers[question.correct_answer];
		});
		const data = {
			time_limit: timeLimit,
			questions: questions
		};
		console.log(data);
		const res = await fetch(import.meta.env.VITE_URL + 'create_room', {
			method: 'POST',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(data)
		});
		const resJson = await res.json();
		goto('/lobby/' + resJson.room_code);
	};
</script>

<svelte:head>
	<title>Create a Game</title>
	<meta name="description" content="Create a Game" />
</svelte:head>

<div class="main">
	<div class="top-bar">
		<div>
			<label for="time-limit">Time Limit:</label>
			<input type="number" bind:value={timeLimit} id="time-limit" />
			<span>seconds</span>
		</div>
		<div>
			<button class="create-button" on:click={createGame}>Create</button>
		</div>
	</div>
	<div class="questions">
		{#each questions as question, i (i)}
			<div class="question-container">
				<div class="question-input">
					<label for="question{i}">{i + 1})</label>
					<input type="text" id="question{i}" bind:value={question.prompt} />
				</div>
				<div class="answer-container">
					{#each question.answers as choice, j (j)}
						<div class="answer-input">
							<input type="text" id="choice{i}-{j}" bind:value={choice} />
							<input
								type="radio"
								bind:group={question.correct_answer}
								value={j}
								checked={question.correct_answer === choice}
							/>
						</div>
					{/each}
				</div>
			</div>
		{/each}
	</div>
	<div class="bottom-bar">
		<button on:click={addQuestion}> Add Question </button>
		<button on:click={removeQuestion}> Remove Question </button>
	</div>
</div>

<style>
	div.main {
		display: flex;
		flex-direction: column;
		width: 60%;
		height: 80vh;
		margin: 2em auto;
		padding: 2em;
		background: white;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
		border-radius: 15px;
		box-sizing: border-box;
	}

	div.top-bar {
		display: flex;
		justify-content: space-between;
		align-items: center;
		height: 40px;
		padding: 10px 30%;
		font-size: 20px;
	}

	div.top-bar label {
		font-size: 25px;
	}

	div.top-bar input {
		width: 40px;
		text-align: center;
		margin: 0 10px;
		appearance: auto;
		-webkit-appearance: none;
		-moz-appearance: textfield;
	}

	div.questions {
		flex: 1;
		margin: 2em 0;
		overflow-y: auto;
		display: flex;
		flex-direction: column;
		border: 2px solid rgb(131, 131, 131);
		border-radius: 15px;
	}

	div.bottom-bar {
		display: flex;
		justify-content: space-between;
		height: 40px;
		padding: 10px 30%;
		font-size: 20px;
	}

	div.question-container {
		padding: 1em;
		display: flex;
		flex-direction: column;
	}

	div.question-input,
	div.answer-container {
		display: flex;
		flex-direction: row;
		align-items: center;
		margin-bottom: 10px;
	}

	div.question-input label {
		padding: 0 0.5em;
		font-size: 20px;
	}

	div.answer-container {
		flex-wrap: wrap;
	}

	div.answer-input {
		flex: 0 0 45%;
		display: flex;
		margin: 1em;
	}

	div.question-container input {
		flex: 1;
	}

	div.answer-input input[type='text'] {
		flex: 0 0 80%;
	}

	div.answer-input input[type='radio'] {
		flex: 1;
	}

	input[type='text'] {
		padding: 10px;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
	}
</style>
