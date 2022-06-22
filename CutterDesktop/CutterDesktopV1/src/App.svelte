<script>

  import { fade, fly } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/tauri'
  import styles from './assets/styles.css'
  import { appWindow } from '@tauri-apps/api/window'
  import { onMount } from 'svelte';

  import { Tabs, Tab, TabList, TabPanel } from 'svelte-tabs';
  let clips_data;
  let render_script="";
  let theme="Dark";

  function theme_button() {

  }
  function define_render_script(data) {
    render_script=data;
  }
  function select_dir() {

    invoke('directory_select');
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
  let render_object;
  let out_script;
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
      out_html = out_html + "<td><input value=" + data[a]["comment"]+ "></input></td>\n";
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
  .getElementById('titlebar-maximize')
  .addEventListener('click', () => appWindow.toggleMaximize())
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
  <div class="titlebar-button" id="titlebar-maximize">
    <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="maximize"
    />
  </div>
  <div class="titlebar-button" id="titlebar-close">
    <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>


  <button on:click={theme_button} class="theme_button"> {theme}</button>
</div>
</nav>
<main>


<Tabs>
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



  
  
    
  
    
   

  


</main>

<style>
  :root {
    border-radius: 20px;
    border-color: white;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
      Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  }

  main {
    text-align: center;
    padding: 1em;
    margin: 0 auto;
    border-radius: 20px;
    border-color: white;
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
