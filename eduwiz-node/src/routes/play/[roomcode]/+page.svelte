<script lang="ts">
    import type { RoomCode } from './+page';
    export let data: RoomCode;
</script>

<svelte:head>
  <title>Room {data.roomCode}</title>
  <meta name="description" content="Game Room" />
</svelte:head>

<p>Welcome to room {data.roomCode}</p>