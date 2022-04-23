<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import SvelteTable from "svelte-table";
  let final_html;
  let render_object;

  function select_file() {
    invoke("select_file_dialog").then((message) => save_in_store(message));
  }
  function save_in_store(data) {
    render_object=data;
    recive_data(data);
  }
  function recive_data(data) {
    let out_html = `<table>
  <tr>
    <th>Comment</th>
    <th>Start Time</th>
    <th>End Time</th>
    <th>Filename</th>
  </tr> \n`;
    for (var a = 0; a < data.length; a++) {
      out_html = out_html + "<tr>\n";
      out_html = out_html + "<td>" + data[a]["comment"]+ "</td>\n";
      out_html = out_html + "<td>" + data[a]["start_time"] + "</td>\n";
      out_html = out_html + "<td>" + data[a]["end_time"] + "</td>\n";
      out_html = out_html + "<td>" + data[a]["file_belong"] + "</td>\n";
      out_html = out_html + "</tr>\n";
    }
    out_html = out_html + "<table>";
    final_html=out_html;
    return out_html;
  }
   
   function bind_inputs() {

   }

   function render() {
    invoke('render', { input: render_object })
   }
  
</script>

<main>
  <button on:click={select_file}> Select file </button>
<button on:click={render}> Render  </button>
  <div class="table">
    {@html final_html}
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
