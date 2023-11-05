<script lang="ts">
	let questions: any[] = [];
	let timeLimit: number = 60;
	const createGame = async () => {
		questions.forEach((question) => {
			question.correct_answer = question.answers[question.correct_answer];
		});
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
			<input type="text" value="60" id="time-limit" />
			<span>seconds</span>
		</div>
		<div>
			<button class="create-button" on:click={createGame}>Create</button>
		</div>
	</div>
	<div class="questions">
		{#each questions as question, i (i)}
			<div class="question-container">
				<label for="question{i}">Question {i + 1}:</label>
				<input type="text" id="question{i}" bind:value={question.prompt} />
				{#each question.answers as choice, j (j)}
					<div class="answer-container">
						<label for="choice{i}-{j}">Choice {j + 1}:</label>
						<input type="text" id="choice{i}-{j}" bind:value={choice} />
						<label>
							<input
								type="radio"
								bind:group={question.correct_answer}
								value={j}
								checked={question.correct_answer === choice}
							/>
							Correct Answer
						</label>
					</div>
				{/each}
			</div>
		{/each}
		<button
			on:click={() => {
				questions.push({
					prompt: '',
					answers: ['', '', '', ''],
					correct_answer: ''
				});
				questions = questions;
			}}
		>
			Add Question
		</button>
		<button
			on:click={() => {
				questions.pop();
				questions = questions;
				console.log(questions);
			}}
		>
			Remove Question
		</button>
	</div>
</div>

<style>
	div.main {
		display: flex;
		flex-direction: column;
		width: 60%;
		height: 90vh;
		margin: 0 auto;
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
	}

	div.questions {
		/* flex-grow: 5; */
		overflow-y: auto;
	}
</style>
