<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { Tabs, Tab, TabList, TabPanel } from 'svelte-tabs';
  import Table from './Table.svelte';
  import Compile from './Compile.svelte';
  import Select_dir from './Select_Dir.svelte';
	import {onMount} from 'svelte';
  import { appWindow } from '@tauri-apps/api/window'

  let table_on=false;
  let compile_on=false;
  let select_dir_on=false;

  function table_on_function(){
    table_on=!table_on;
    compile_on=false;
    select_dir_on=false;
  }
  function compile_on_function(){
    compile_on=!compile_on;
    select_dir_on=false;
    table_on=false;
  }
  function select_dir_function(){
    select_dir_on=!select_dir_on;
     table_on=false;
     compile_on=false;
  }



  onMount(() => {
    document
  .getElementById('titlebar-minimize')
  .addEventListener('click', () => appWindow.minimize())
document
  .getElementById('titlebar-maximize')
  .addEventListener('click', () => appWindow.toggleMaximize())
document
  .getElementById('titlebar-close')
  .addEventListener('click', () => appWindow.close())
	});
</script>



  <div data-tauri-drag-region class="titlebar">
    <div class="buttons"  >
    <button  class="control_buttons" on:click={table_on_function} > Splitted Table </button>
    <button  class="control_buttons" on:click={compile_on_function} > Results</button>
    <button   class="control_buttons" on:click={select_dir_function} > Select Area</button>
  </div>
    <div class="titlebar-button" id="titlebar-minimize">
      <img
        src="https://api.iconify.design/mdi:window-minimize.svg"
        alt="minimize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-maximize">
      <img
        src="https://api.iconify.design/mdi:window-maximize.svg"
        alt="maximize"
      />
    </div>
    <div class="titlebar-button" id="titlebar-close">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
    </div>

 
  
  </div>
  <main>
<div class="main">
  {#if table_on}
	
<Table></Table>
  {/if}

  {#if compile_on}
	
  <Compile></Compile>
    {/if}

    {#if select_dir_on}
	
    <Select_dir></Select_dir>
      {/if}
</div>
  
</main>


<style>
  .buttons {
    position: relative;
    right:25%;
  }
  .main {
    margin-top: 50px;
  }
  .control_buttons {
    color:white;
    font-size: 16px;
    font-family: 'Courier New', Courier, monospace;
    font-weight: bold;
    border-color: white;
    padding-left: 10px;
    margin-bottom: 5px;
    margin-left: 5px;
    background: transparent;
    border-style: none none solid none;
  }
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
</style>
