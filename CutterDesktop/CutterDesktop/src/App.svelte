<script>
import { invoke } from '@tauri-apps/api/tauri'
import {Table} from './Table.svelte';
// With the Tauri global script, enabled when `tauri.conf.json > build > withGlobalTauri` is set to true:
	export let name;
	let  html;
	function select_file() {
		invoke('select_file_dialog').then((message) => save_in_store(message))

	}
    function save_in_store(data) {
html=data;
    }
	



</script>

<main>
	<button on:click={select_file}> select file </button>

    
    <div class="table">
 	
    </div>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>
