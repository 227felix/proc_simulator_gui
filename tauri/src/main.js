const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const MAX_KEY_LENGTH = 8;
let num_rep = "hex";
let max_digits_hex = 8;
let max_digits_bin = 32;
let max_digits_dec = 10;
let autoclockInterval;
let autoclockIntervalTime = 30;

const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();

document
  .getElementById("titlebar-minimize")
  ?.addEventListener("click", () => appWindow.minimize());
document
  .getElementById("titlebar-maximize")
  ?.addEventListener("click", () => appWindow.toggleMaximize());
document
  .getElementById("titlebar-close")
  ?.addEventListener("click", () => appWindow.close());

// end titlebar

async function clock() {
  let new_state = await invoke("clock_processor", {});
  return new_state;
}

function render_table(table_id, data) {
  let table = document.querySelector(`#${table_id}`);
  // destroy all children
  while (table.firstChild) {
    table.removeChild(table.firstChild);
  }
  // create a new head
  let tr_head = document.createElement("tr");
  let th_key = document.createElement("th");
  let th_value = document.createElement("th");
  th_key.textContent = "Key";
  th_value.textContent = "Value";
  tr_head.appendChild(th_key);
  tr_head.appendChild(th_value);
  table.appendChild(tr_head);

  for (let key in data) {
    let value = data[key];

    let tr = document.createElement("tr");
    tr.setAttribute("data-key", key);
    let td_key = document.createElement("td");
    let td_value = document.createElement("td");
    td_key.textContent = key;
    td_value.textContent = value;
    tr.appendChild(td_key);
    tr.appendChild(td_value);
    table.appendChild(tr);
  }
}

function render_table_with_fragment(table_id, data) {
  let table = document.querySelector(`#${table_id}`);
  // destroy all children
  while (table.firstChild) {
    table.removeChild(table.firstChild);
  }
  // create a new head
  let tr_head = document.createElement("tr");
  let th_key = document.createElement("th");
  let th_value = document.createElement("th");
  th_key.textContent = "Key";
  th_value.textContent = "Value";
  tr_head.appendChild(th_key);
  tr_head.appendChild(th_value);
  table.appendChild(tr_head);

  let fragment = document.createDocumentFragment();
  for (let key in data) {
    let value = data[key];

    let tr = document.createElement("tr");
    tr.setAttribute("data-key", key);
    let td_key = document.createElement("td");
    let td_value = document.createElement("td");
    td_key.textContent = key;
    td_value.textContent = value;
    tr.appendChild(td_key);
    tr.appendChild(td_value);
    fragment.appendChild(tr);
  }
  table.appendChild(fragment);
}

function update_table(table_id, data) {
  let table = document.querySelector(`#${table_id}`);
  for (let key in data) {
    let value = data[key];
    let row = table.querySelector(`tr[data-key="${key}"]`);
    if (row) {
      let td_value = row.querySelector("td:last-child");
      td_value.textContent = value;
    } else {
      let tr = document.createElement("tr");
      tr.setAttribute("data-key", key);
      let td_key = document.createElement("td");
      let td_value = document.createElement("td");
      td_key.textContent = key;
      td_value.textContent = value;
      tr.appendChild(td_key);
      tr.appendChild(td_value);
      table.appendChild(tr);
    }
  }
}

function render_fetch(fetch) {
  render_table("fetch-table", fetch);
}

function render_decode(decode) {
  let decode_copy = { ...decode };
  delete decode_copy.reg_bank;
  render_table("decode-table", decode_copy);
}

function render_execute(execute) {
  render_table("execute-table", execute);
}

function render_memory(memory) {
  render_table("memory-table", memory);
}

function render_write_back(write_back) {
  render_table("write-back-table", write_back);
}

function render_rom(rom, fetch_pc) {
  render_table_with_fragment("rom-table", rom);
  let base;
  switch (num_rep) {
    case "hex":
      base = 16;
      break;
    case "bin":
      base = 2;
      break;
    case "dec":
      base = 10;
      break;
  }
  let parsed_fetch_pc = parseInt(fetch_pc, base);
  let table = document.querySelector("#rom-table");
  let row = table.querySelector(`tr[data-key="${parsed_fetch_pc}"]`);
  console.log(fetch_pc);
  console.log(row);
  if (row) {
    row.id = "highlighted";
    row.scrollIntoView({
      behavior: "auto",
      block: "center",
      inline: "center",
    });
  }
}

function render_ram(ram) {
  render_table_with_fragment("ram-table", ram);
}

function render_reg_bank(reg_bank) {
  render_table("reg-table", reg_bank);
}

function update_fetch(fetch) {
  update_table("fetch-table", fetch);
}

function update_decode(decode) {
  let decode_copy = { ...decode };
  delete decode_copy.reg_bank;
  update_table("decode-table", decode_copy);
}

function update_execute(execute) {
  update_table("execute-table", execute);
}

function update_memory(memory) {
  update_table("memory-table", memory);
}

function update_write_back(write_back) {
  update_table("write-back-table", write_back);
}

function update_rom(rom, fetch_pc) {
  let table = document.querySelector("#rom-table");
  // grab all tr with id=highlighted and remove the id
  let highlighted = table.querySelector("#highlighted");
  if (highlighted) {
    highlighted.removeAttribute("id");
  }

  let row = table.querySelector(`tr[data-key="${fetch_pc}"]`);
  if (row) {
    let td_value = row.querySelector("td:last-child");
    row.id = "highlighted";
    row.scrollIntoView({
      behavior: "auto",
      block: "center",
      inline: "center",
    });
  }
}

function update_ram(changed_ramfield) {
  // update the ram field that has changed
  // check if the changed field holds -1 as twos complement and skip if so FIXME:
  // parse the key with the number representation
  let base;
  switch (num_rep) {
    case "hex":
      base = 16;
      break;
    case "bin":
      base = 2;
      break;
    case "dec":
      base = 10;
      break;
  }

  let key = parseInt(changed_ramfield[0], base);
  if (key == -1) {
    return;
  }

  let value = changed_ramfield[1];
  let ram_field = document.querySelector(`#ram-table tr[data-key="${key}"]`);

  if (ram_field) {
    let td_value = ram_field.querySelector("td:last-child");
    td_value.textContent = value;
  }

  console.log(ram_field);
}

function update_reg_bank(reg_bank) {
  update_table("reg-table", reg_bank);
}

function render_state(state, first_render = false) {
  let fetch = state.fetch;
  let fetch_pc = state.fetch.pc;
  let decode = state.decode;
  let reg_bank = state.decode.reg_bank;
  let execute = state.execute;
  let memory = state.memory;
  let write_back = state.write_back;
  let rom = state.rom;
  let ram = state.ram;

  render_fetch(fetch);
  render_decode(decode);
  render_execute(execute);
  render_memory(memory);
  render_write_back(write_back);
  render_rom(rom, fetch_pc);
  render_ram(ram);
  render_reg_bank(reg_bank);
}

function update_state(state) {
  let fetch = state.fetch;
  let decode = state.decode;
  let reg_bank = state.decode.reg_bank;
  let execute = state.execute;
  let memory = state.memory;
  let write_back = state.write_back;
  let rom = state.rom;
  let changed_ramfield = state.changed_ramfield;

  let base = 2;
  switch (num_rep) {
    case "hex":
      base = 16;
      break;
    case "bin":
      base = 2;
      break;
    case "dec":
      base = 10;
      break;
  }
  let fetch_pc = parseInt(state.fetch.pc, base);

  update_fetch(fetch);
  update_decode(decode);
  update_execute(execute);
  update_memory(memory);
  update_write_back(write_back);
  update_rom(rom, fetch_pc);
  update_ram(changed_ramfield);
  update_reg_bank(reg_bank);
}

async function set_num_representation(representation) {
  await invoke("set_num_representation", { representation });
  let new_state = await invoke("get_state", {});
  let new_state_obj = JSON.parse(new_state);
  render_state(new_state_obj);
}

function startAutoclock() {
  autoclockInterval = setInterval(async () => {
    let new_state = await clock();
    let new_state_obj = JSON.parse(new_state);
    update_state(new_state_obj);
  }, autoclockIntervalTime);
}

function stopAutoclock() {
  clearInterval(autoclockInterval);
}

window.addEventListener("DOMContentLoaded", async () => {
  let new_state = await invoke("get_state", {}).then((new_state) => {
    let new_state_obj = JSON.parse(new_state);

    let filepath = new_state_obj.file_path;
    let filepath_span = document.querySelector("#filepath-div");
    filepath_span.textContent = "Rom: ";
    filepath_span.textContent += filepath;

    render_state(new_state_obj, true);
  });

  let clock_button = document.querySelector("#clock-button");
  clock_button.addEventListener("click", async (e) => {
    let proc_div = document.querySelector("#proc-div");
    let new_state = clock().then((new_state) => {
      //console.log(new_state);
      let new_state_obj = JSON.parse(new_state);
      update_state(new_state_obj);
    });
  });

  let representation_select = document.querySelector("#representation-buttons");
  representation_select.addEventListener("change", async (e) => {
    if (e.target.name === "num-representation") {
      let representation = e.target.value;
      num_rep = representation;
      await set_num_representation(representation).then(() => {
        console.log(num_rep);
      });
    }
  });

  let reset_button = document.querySelector("#reset-button");
  reset_button.addEventListener("click", async (e) => {
    let new_state = await invoke("reset_processor", {});
    let new_state_obj = JSON.parse(new_state);
    render_state(new_state_obj);
  });

  let load_button = document.querySelector("#load-button");
  load_button.addEventListener("click", async (e) => {
    let new_state = await invoke("load_program", {});
    let new_state_obj = JSON.parse(new_state);
    let filepath = new_state_obj.file_path;
    let filepath_span = document.querySelector("#filepath-div");
    filepath_span.textContent = "Rom: ";
    filepath_span.textContent += filepath;
    render_state(new_state_obj);
  });

  let reload_button = document.querySelector("#reload-button");
  reload_button.addEventListener("click", async (e) => {
    let new_state = await invoke("reload_program", {});
    let new_state_obj = JSON.parse(new_state);
    let filepath = new_state_obj.file_path;
    let filepath_span = document.querySelector("#filepath-div");
    filepath_span.textContent = "Rom: ";
    filepath_span.textContent += filepath;
    render_state(new_state_obj);
  });

  let autoclock_button = document.querySelector("#autoclock-button");
  let stop_button = document.querySelector("#stop-button");
  let autoclock_interval_input = document.querySelector("#autoclock-interval");

  autoclock_button.addEventListener("click", (e) => {
    autoclockIntervalTime = parseInt(autoclock_interval_input.value) || 30;
    startAutoclock();
    autoclock_button.style.display = "none";
    stop_button.style.display = "inline-block";
  });

  stop_button.addEventListener("click", (e) => {
    stopAutoclock();
    stop_button.style.display = "none";
    autoclock_button.style.display = "inline-block";
  });
  document.addEventListener("contextmenu", (event) => event.preventDefault());
});
