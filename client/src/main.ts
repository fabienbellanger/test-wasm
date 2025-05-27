import './assets/css/style.css'
import init, { add, greet, setup, Task } from '../pkg/test-wasm'

async function init_wasm() {
    await init()

    // Setup WASM library
    setup()

    const btn = document.getElementById('msg-btn')
    const input = document.getElementById('msg-input') as HTMLInputElement
    const messageElem = document.getElementById('msg')
    if (btn && input && messageElem) {
        btn.onclick = () => {
            messageElem.textContent = greet(input.value)
        }
    }

    // Test the add function
    const addResult = document.getElementById('add-result')
    const addbtn = document.getElementById('add-btn')
    if (addResult && addbtn) {
        addbtn.onclick = () => {
            addResult.textContent = `2 + 3 = ${add(2, 3)}`
        }
    }

    // Test the TaskClass
    const task = new Task('Test Task')
    console.log('Task created', task)
    try {
        task.id = '12345'
    } catch (error) {
        console.error(error)
    }
    task.name = 'Updated Task Name'
    console.log(`Task updated: ${task.name}`)
}

;(async () => {
    try {
        await init_wasm()
    } catch (error) {
        console.error('Error initializing WASM:', error)
    }
})()
