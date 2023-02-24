<script>
  import { fade, fly } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/tauri'
  import styles from './assets/styles.css'
  import { appWindow } from '@tauri-apps/api/window'
  import { onMount } from 'svelte';
  import { Tabs, Tab, TabList, TabPanel } from 'svelte-tabs';
  import Switch from './lib/Switch.svelte';

  let clips_data;
  let render_script="";
  let is_theme_dark;
  let current_theme="Light";

  function NewTab() {
    alert("Read more:https://github.com/alex5250/VideoEdit-Long-Files");
   }

  function define_render_script(data) {
    render_script=data;
  }
  function select_dir() {
    invoke('directory_select');
  }

  function change_theme() {
    is_theme_dark=!is_theme_dark;
    if( is_theme_dark==true) {
      current_theme="Dark"
      
    }
    else {
      current_theme="Light"
    }
  }

  function save() {
invoke('save', { input: render_script })
}

  function render() {
    invoke('render', { input: clips_data }).then((message) => define_render_script(message))
  }
  function parse_file() {
     invoke('table_render').then((message) => recive_data(message))
  }
  let final_html;
    function recive_data(data) {
    clips_data=data;
    let out_html = `<table>
  <tr>
  <thead>
    <th>Comment</th>
    <th>Start Time</th>
    <th>End Time</th>
    <th>Filename</th>
    </thead>
  </tr> \n
  <style>
  
</style>
`;
    for (var a = 0; a < data.length; a++) {
      out_html = out_html + "<tr>\n";
      out_html = out_html + "<td><input class='edit' value=" + data[a]["comment"]+ "></input></td>\n";
      out_html = out_html + "<td>" + data[a]["start_time"] + "</td>\n";
      out_html = out_html + "<td>" + data[a]["end_time"] + "</td>\n";
      out_html = out_html + "<td>" + data[a]["file_belong"] + "</td>\n";
      out_html = out_html + "</tr>\n";
    }
    out_html = out_html + "<table>";
    final_html=out_html;
    return out_html;
  }
  onMount(async () => {
    document
  .getElementById('titlebar-minimize')
  .addEventListener('click', () => appWindow.minimize())

document
  .getElementById('titlebar-close')
  .addEventListener('click', () => appWindow.close())
	});
</script>
<nav id="sidebar" class="visible">
 
<div data-tauri-drag-region class="titlebar">
  
  <div class="titlebar-button" id="titlebar-minimize">
    <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
    />
  </div>
 


  <div class="titlebar-button" id="titlebar-close">
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
  <button class="theme_button left" on:click={NewTab}>About </button>
  <button class="theme_button right " on:click={change_theme}>{current_theme} </button>

</div>
</nav>
<main >

  <div   class="main_panel" id="body">
<Tabs >
  <TabList>
    <Tab>Select dir</Tab>
    <Tab>Process into table</Tab>
    <Tab>Render</Tab>
  </TabList>
  <TabPanel>
    <div class="welcome" transition:fly="{{ y: 100, duration: 2000 }}">
      <button class="select_button" on:click={select_dir}>Select Dir</button>
      <p> Select and directory where is your media files placed.</p>
   </div>
  </TabPanel>
  <TabPanel>
    <div class="table" transition:fly="{{ y: 100, duration: 2000 }}" on:load="{parse_file}">
      <button class="select_button" on:click={parse_file}>Process an Table</button>
      <div class="table" on:click={render}>
        {@html final_html}
      </div>
  </TabPanel>
  <TabPanel on:click={render}>
    <div class="render">
      <textarea bind:value={render_script} ></textarea>
     </div> 
     <button class="select_button" on:click={save}>Save</button>
    </TabPanel>
</Tabs>

{#if is_theme_dark}
	<style>
		body {
			background-color: #363732;
		}
    .titlebar {
      background-color: #363732;
    }
    
    p,h1 {
      color:white;
    }
	</style>
{:else if !is_theme_dark}
	<style>
   
		body {
   
			background-color: #e6e6e6;
		}
    .titlebar {
      background-color: #e6e6e6;
    }
    p,h1 {
      color:black;
    }

   
	</style>
{/if}
</div>
</main>