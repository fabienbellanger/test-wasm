import './assets/css/style.css'
import init, * as wasm from '../pkg/test-wasm'

async function init_wasm() {
    await init()

    // Setup WASM library
    wasm.setup()

    const btn = document.getElementById('msg-btn')
    const input = document.getElementById('msg-input') as HTMLInputElement
    const messageElem = document.getElementById('msg')
    if (btn && input && messageElem) {
        btn.onclick = () => {
            messageElem.textContent = wasm.greet(input.value)
        }
    }

    // Test the add function
    const addResult = document.getElementById('add-result')
    const addbtn = document.getElementById('add-btn')
    if (addResult && addbtn) {
        addbtn.onclick = () => {
            addResult.textContent = `2 + 3 = ${wasm.add(2, 3)}`
        }
    }

    // Test the TaskClass
    tasksDemo()
}

/**
 * Demo for the Task class
 *
 */
function tasksDemo() {
    const addTaskInput = document.getElementById(
        'add-task-input',
    ) as HTMLInputElement
    const addTaskBtn = document.getElementById('add-task-btn')
    const tasksDiv = document.getElementById('tasks')
    if (addTaskBtn && addTaskInput && tasksDiv) {
        const tasks = new wasm.Tasks()

        addTaskBtn.onclick = () => {
            if (addTaskInput.value.length > 0) {
                const task = new wasm.Task(addTaskInput.value)
                // try {
                //     task.id = '12345'
                // } catch (error) {
                //     console.error(error)
                // }
                // task.name = addTaskInput.value
                // console.log(`Task updated: ${task.name}`)

                tasks.add(task)
                addTaskInput.value = ''
                console.log('Tasks', tasks.list())
                // const lastTask = tasks.pop()
                // if (lastTask) {
                //     console.log('Removed task', lastTask)
                // }
                // console.log('Tasks', tasks.list())

                tasksDiv.innerHTML += `<div>- ${task.name}</div>`
            }
        }
    }
}

;(async () => {
    try {
        await init_wasm()
    } catch (error) {
        console.error('Error initializing WASM:', error)
    }
})()
