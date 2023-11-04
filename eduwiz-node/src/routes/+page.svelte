<script>
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { logIn, user } from '$lib/flow/utils';

  onMount(() => {
    if ($user.loggedIn) {
      goto('/home');
    }
  });
</script>

<svelte:head>
  <title>Login</title>
  <meta name="description" content="Login page" />
</svelte:head>

<style>
  .login-container {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #000080; /* dark blue */
  }

  .login-box {
    width: 400px;
    padding: 2em;
    background: white;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    border-radius: 15px;
    text-align: center;
    box-sizing: border-box;
  }

  img {
    width: 100%;
    margin-bottom: 1em;
  }

  .buttons button {
    width: 100%;
    padding: 10px 0;
    margin-top: 1em;
  }
</style>

<div class="login-container">
  <div class="login-box">
    <img src="logo-words.png" alt="logo" />
    <h2>Please log in to continue</h2>
    <div class="buttons">
      <button on:click={logIn}>Log In</button>
    </div>
  </div>
</div>
