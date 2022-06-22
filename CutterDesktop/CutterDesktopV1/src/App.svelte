<script>

  import { fade, fly } from 'svelte/transition';
  import { invoke } from '@tauri-apps/api/tauri'
  import styles from './assets/styles.css'

  let select_dir_visibility=true;
  let table_dir_visibility=false;
  let table_dir_visibility_button=false;
  let render_area_visibility=false;
  let clips_data;
  let render_script="";

  function define_render_script(data) {
    render_script=data;
  }
  function select_dir() {
    select_dir_visibility=false;
    invoke('directory_select');
    table_dir_visibility=true;
    table_dir_visibility_button=true;
  }


  function render() {
    render_area_visibility=true;
    table_dir_visibility=false;
    table_dir_visibility_button=false;
    invoke('render', { input: clips_data }).then((message) => define_render_script(message))

  }
  function parse_file() {
     invoke('table_render').then((message) => recive_data(message))
     table_dir_visibility_button=false;

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
</script>

<main>
  {#if select_dir_visibility}

  <div class="welcome" transition:fly="{{ y: 100, duration: 2000 }}">
     <button class="select_button" on:click={select_dir}>Select Dir</button>
     <p> Select and directory where is your media files placed.</p>
  </div>
  
  {/if}


  <div class="table" transition:fly="{{ y: 100, duration: 2000 }}" on:load="{parse_file}">
  
    <button class="select_button" on:click={parse_file}>Process an Table</button>
 

    <div class="table" on:click={render}>
      {@html final_html}
    </div>

   
    {#if render_area_visibility}
    
    <div class="render">
     <textarea bind:value={render_script}></textarea>
    </div>
  {/if}
  
  </div>

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
