const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;

async function clock() {
  let new_state = await invoke("clock_processor", {});
  return new_state;
}

function update_fetch(fetch) {
  let fetch_div = document.querySelector("#fetch-div");
  fetch_div.innerHTML = "";
  for (let key in fetch) {
    let value = fetch[key];
    fetch_div.innerHTML += `${key}: ${value} <br>`;
  }
}

function update_decode(decode) {
  let decode_div = document.querySelector("#decode-div");
  decode_div.innerHTML = "";
  for (let key in decode) {
    if (key === "reg_bank") {
      continue;
    }
    let value = decode[key];
    decode_div.innerHTML += `${key}: ${value} <br>`;
  }
}

function update_execute(execute) {
  let execute_div = document.querySelector("#execute-div");
  execute_div.innerHTML = "";
  for (let key in execute) {
    let value = execute[key];
    execute_div.innerHTML += `${key}: ${value} <br>`;
  }
}

function update_memory(memory) {
  let memory_div = document.querySelector("#memory-div");
  memory_div.innerHTML = "";
  for (let key in memory) {
    let value = memory[key];
    memory_div.innerHTML += `${key}: ${value} <br>`;
  }
}

function update_write_back(write_back) {
  let write_back_div = document.querySelector("#write-back-div");
  write_back_div.innerHTML = "";
  for (let key in write_back) {
    let value = write_back[key];
    write_back_div.innerHTML += `${key}: ${value} <br>`;
  }
}

function update_rom(rom) {
  let rom_div = document.querySelector("#rom-div");
  rom_div.innerHTML = ""; // Leeren des Containers

  let fragment = document.createDocumentFragment();
  for (let key in rom) {
    let value = rom[key];
    let span = document.createElement("span");
    span.textContent = `${key}: ${value}`;
    span.style.display = "block"; // Für eine zeilenweise Darstellung
    fragment.appendChild(span);
  }
  rom_div.appendChild(fragment); // Fragment auf einmal hinzufügen
}

function update_ram(ram) {
  let ram_div = document.querySelector("#ram-div");
  ram_div.innerHTML = ""; // Leeren des Containers

  let fragment = document.createDocumentFragment();
  for (let key in ram) {
    let value = ram[key];
    let span = document.createElement("span");
    span.textContent = `${key}: ${value}`;
    span.style.display = "block"; // Für eine zeilenweise Darstellung
    fragment.appendChild(span);
  }
  ram_div.appendChild(fragment); // Fragment auf einmal hinzufügen
}

function update_reg_bank(reg_bank) {
  let reg_bank_div = document.querySelector("#register-div");
  reg_bank_div.innerHTML = "";
  for (let key in reg_bank) {
    let value = reg_bank[key];
    reg_bank_div.innerHTML += `${key}: ${value} <br>`;
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

  update_fetch(fetch);
  update_decode(decode);
  update_execute(execute);
  update_memory(memory);
  update_write_back(write_back);
  update_rom(rom);
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

    update_state(new_state_obj);
  });

  let clock_button = document.querySelector("#clock-button");
  clock_button.addEventListener("click", async (e) => {
    let proc_div = document.querySelector("#proc-div");
    let new_state = clock().then((new_state) => {
      console.log(new_state);
      // proc_div.innerHTML = new_state;
      // deserialize the new_state
      let new_state_obj = JSON.parse(new_state);
      update_state(new_state_obj);
    });
  });

  let representation_select = document.querySelector("#representation-buttons");
  representation_select.addEventListener("change", async (e) => {
    if (e.target.name === "num-representation") {
      let representation = e.target.value;
      await set_num_representation(representation);
    }
  });
});
