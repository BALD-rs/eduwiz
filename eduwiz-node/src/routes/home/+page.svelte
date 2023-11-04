<script>
    import { goto } from '$app/navigation';
	import { logOut } from '$lib/flow/utils';

    let roomCode = '';

    function joinRoom() {
        goto(`/live/${roomCode}`);
    }
</script>
<svelte:head>
  <title>Home</title>
  <meta name="description" content="Homepage" />
</svelte:head>

<style>
body,
html {
  height: 100%;
  display: grid;
  background-color: #1e1e2e;
  color: #cdd6f4;
  font-family: 'Noto Sans', sans-serif;
}
</style>

<div class="topnav">
  <div class="input">
    <input type="text" placeholder="Enter room code" bind:value={roomCode} />
    <button on:click={joinRoom} class="button">Join Room</button>
    <button on:click={logOut} class="button">Logout</button>
  </div>
</div>