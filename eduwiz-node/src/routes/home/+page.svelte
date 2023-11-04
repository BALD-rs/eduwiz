<script>
    import { goto } from '$app/navigation';

    let username = 'joe biden';
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
  /* Add your custom styles here */
  .topnav {
    background-color: #333;
    overflow: hidden;
  }

  .topnav a {
    float: left;
    color: #f2f2f2;
    text-align: center;
    padding: 14px 16px;
    text-decoration: none;
    font-size: 17px;
  }

  .topnav a:hover {
    background-color: #ddd;
    color: black;
  }

  .topnav a.active {
    background-color: #4caf50;
    color: white;
  }

  .input {
    display: inline-block;
    margin: 10px;
  }

  .button {
    padding: 10px 20px;
    background-color: #4caf50;
    color: white;
    border: none;
    border-radius: 5px;
    cursor: pointer;
  }
</style>

<div class="topnav">
  <a class="active" href="#home">EduWiz</a>
  <a href="/profile">Hello, {username}</a>
  <div class="input">
    <input type="text" placeholder="Enter room code" bind:value={roomCode} />
    <button on:click={joinRoom} class="button">Join Room</button>
  </div>
</div>