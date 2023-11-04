<script>
  import { logIn, logOut, user } from '$lib/flow/utils';
</script>

<svelte:head>
  <title>Login</title>
  <meta name="description" content="Login page" />
</svelte:head>

<section>
  <h1>EduWiz</h1>
  {#if $user.loggedIn}
    <h2>Welcome, {$user.addr}!</h2>
    <div class="buttons">
      <button on:click={logOut}>Log Out</button>
    </div>
  {:else}
    <h2>Please log in to continue</h2>
    <div class="buttons">
      <button on:click={logIn}>Log In</button>
    </div>
  {/if}
</section>

<style>
  section {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #f5f5f5;
  }

  h1,
  h2 {
    color: #333;
  }

  .buttons {
    margin-top: 20px;
  }

  button {
    margin: 0 10px;
    padding: 10px 20px;
    font-size: 1em;
    border: none;
    border-radius: 5px;
    color: #fff;
    background-color: #007bff;
    cursor: pointer;
    transition: background-color 0.3s;
  }

  button:hover {
    background-color: #0056b3;
  }

  button:disabled {
    background-color: #aaa;
  }

  /* Add any additional styling for new elements here */
</style>
