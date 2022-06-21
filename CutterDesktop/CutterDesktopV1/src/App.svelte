<script>
  import logo from './assets/svelte.png'
  import Counter from './lib/Counter.svelte'
  import styles from './assets/styles.css'
  import { fade, fly } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/tauri'
  let select_dir_visibility=true;

  function select_dir() {
    select_dir_visibility=false;
    invoke('directory_select');
  }
</script>

<main>
  {#if select_dir_visibility}

  <div class="welcome" transition:fly="{{ y: 0, duration: 0 }}">
     <button class="select_button" on:click={select_dir}>Select Dir</button>
     <p> Select and directory where is your media files placed.</p>
  </div>
  
  {/if}
</main>

<style>
  :root {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
  }

  img {
    height: 16rem;
    width: 16rem;
  }

  h1 {
    color: #ff3e00;
    text-transform: uppercase;
    font-size: 4rem;
    font-weight: 100;
    line-height: 1.1;
    margin: 2rem auto;
    max-width: 14rem;
  }

  p {
    max-width: 14rem;
    margin: 1rem auto;
    line-height: 1.35;
  }

  @media (min-width: 480px) {
    h1 {
      max-width: none;
    }

    p {
      max-width: none;
    }
  }
</style>
