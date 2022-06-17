<script>

    import { invoke } from "@tauri-apps/api/tauri";
  
    let final_html;
    let render_object;
    let out_script;
  
    function save_out_script(data){
    out_script=data;
    }
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
    <thead>
      <th>Comment</th>
      <th>Start Time</th>
      <th>End Time</th>
      <th>Filename</th>
      </thead>
    </tr> \n
  
  
    <style>
  
     table {
      border-collapse: collapse;
      margin: 25px 0;
      font-size: 0.9em;
      font-family: sans-serif;
      min-width: 400px;
      box-shadow: 0 0 20px rgba(0, 0, 0, 0.15);
  }
  
   thead tr {
      background-color: #009879;
      color: #ffffff;
      text-align: left;
  }
  
   th,td {
      padding: 12px 15px;
  }
   tbody tr {
      border-bottom: 1px solid #dddddd;
  }
  
   tbody tr:nth-of-type(even) {
      background-color: #f3f3f3;
  }
  
  tbody tr:last-of-type {
      border-bottom: 2px solid #009879;
  }
  tbody tr.active-row {
      font-weight: bold;
      color: #009879;
  }
  </style>
  `;
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
     
 
  
     function render() {
      invoke('render', { input: render_object }).then((message) => save_out_script(message));
     }
    
  </script>
  
  


  <div class="buttons">
  
    <button on:click={select_file} > Select file </button>
  <button on:click={render}> Render  </button>
  </div>
  
   <div class="table">
      {@html final_html}
    </div>
  
  

  

  
  