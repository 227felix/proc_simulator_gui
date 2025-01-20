const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

const MAX_KEY_LENGTH = 8;
let num_rep = "hex";
let max_digits_hex = 8;
let max_digits_bin = 32;
let max_digits_dec = 10;

async function clock() {
  let new_state = await invoke("clock_processor", {});
  return new_state;
}

function update_table(table_id, data) {
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
    let td_key = document.createElement("td");
    let td_value = document.createElement("td");
    td_key.textContent = key;
    td_value.textContent = value;
    tr.appendChild(td_key);
    tr.appendChild(td_value);
    table.appendChild(tr);
  }
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
  let rom_table = document.querySelector("#rom-table");
  // destroy all children
  while (rom_table.firstChild) {
    rom_table.removeChild(rom_table.firstChild);
  }
  // create a new head
  let tr_head = document.createElement("tr");
  let th_key = document.createElement("th");
  let th_value = document.createElement("th");
  th_key.textContent = "Key";
  th_value.textContent = "Value";
  tr_head.appendChild(th_key);
  tr_head.appendChild(th_value);
  rom_table.appendChild(tr_head);

  for (let key in rom) {
    let value = rom[key];

    let tr = document.createElement("tr");
    let td_key = document.createElement("td");
    let td_value = document.createElement("td");
    td_key.textContent = key;
    td_value.textContent = value;
    tr.appendChild(td_key);
    tr.appendChild(td_value);
    rom_table.appendChild(tr);
    if (key == fetch_pc) {
      tr.id = "highlighted";
      // scroll the highlighted row to the center of the view
      tr.scrollIntoView();
    }
  }
}

function update_ram(ram) {
  let ram_table = document.querySelector("#ram-table");
  // destroy all children
  while (ram_table.firstChild) {
    ram_table.removeChild(ram_table.firstChild);
  }
  // create a new head
  let tr_head = document.createElement("tr");
  let th_key = document.createElement("th");
  let th_value = document.createElement("th");
  th_key.textContent = "Key";
  th_value.textContent = "Value";
  tr_head.appendChild(th_key);
  tr_head.appendChild(th_value);
  ram_table.appendChild(tr_head);

  for (let key in ram) {
    let value = ram[key];

    let tr = document.createElement("tr");
    let td_key = document.createElement("td");
    let td_value = document.createElement("td");
    td_key.textContent = key;
    td_value.textContent = value;
    tr.appendChild(td_key);
    tr.appendChild(td_value);
    ram_table.appendChild(tr);
  }
}

function update_reg_bank(reg_bank) {
  let reg_table = document.querySelector("#reg-table");
  // destroy all children
  while (reg_table.firstChild) {
    reg_table.removeChild(reg_table.firstChild);
  }
  // create a new head
  let tr_head = document.createElement("tr");
  let th_key = document.createElement("th");
  let th_value = document.createElement("th");
  th_key.textContent = "Key";
  th_value.textContent = "Value";
  tr_head.appendChild(th_key);
  tr_head.appendChild(th_value);
  reg_table.appendChild(tr_head);

  for (let key in reg_bank) {
    let value = reg_bank[key];

    let tr = document.createElement("tr");
    let td_key = document.createElement("td");
    let td_value = document.createElement("td");
    td_key.textContent = key;
    td_value.textContent = value;
    tr.appendChild(td_key);
    tr.appendChild(td_value);
    reg_table.appendChild(tr);
  }
}

function update_state(state) {
  let fetch = state.fetch;
  let decode = state.decode;
  let reg_bank = state.decode.reg_bank;
  let execute = state.execute;
  let memory = state.memory;
  let write_back = state.write_back;
  let rom = state.rom;
  let ram = state.ram;

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
  update_ram(ram);
  update_reg_bank(reg_bank);
}

async function set_num_representation(representation) {
  await invoke("set_num_representation", { representation });
  let new_state = await invoke("get_state", {});
  let new_state_obj = JSON.parse(new_state);
  update_state(new_state_obj);
}

window.addEventListener("DOMContentLoaded", async () => {
  let new_state = await invoke("get_state", {}).then((new_state) => {
    let new_state_obj = JSON.parse(new_state);

    let filepath = new_state_obj.file_path;
    let filepath_span = document.querySelector("#filepath-div");
    filepath_span.textContent = "Rom: ";
    filepath_span.textContent += filepath;

    update_state(new_state_obj);
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
    update_state(new_state_obj);
  });

  let load_button = document.querySelector("#load-button");
  load_button.addEventListener("click", async (e) => {
    let new_state = await invoke("load_program", {});
    let new_state_obj = JSON.parse(new_state);
    let filepath = new_state_obj.file_path;
    let filepath_span = document.querySelector("#filepath-div");
    filepath_span.textContent = "Rom: ";
    filepath_span.textContent += filepath;
    update_state(new_state_obj);
  });

  let reload_button = document.querySelector("#reload-button");
  reload_button.addEventListener("click", async (e) => {
    let new_state = await invoke("reload_program", {});
    let new_state_obj = JSON.parse(new_state);
    let filepath = new_state_obj.file_path;
    let filepath_span = document.querySelector("#filepath-div");
    filepath_span.textContent = "Rom: ";
    filepath_span.textContent += filepath;
    update_state(new_state_obj);
  });
});
