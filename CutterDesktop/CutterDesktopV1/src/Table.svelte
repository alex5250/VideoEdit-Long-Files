<script>
    export let data;
    let final_html;
  let render_object;
  let out_script;
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
</script>


<div class="table">
    {@html final_html}
  </div>
