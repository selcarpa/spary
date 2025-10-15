import { reactive } from 'vue'

export interface NotifyOptions {
    color?: string
    timeout?: number
    variant?: string
}

// 通知对象
export interface NotifyItem {
    id: number
    message: string
    color: string
    timeout: number
    variant: string
}

// 队列状态
export const notifyQueue = reactive<NotifyItem[]>([])

// 全局可调用 notify
let nextId = 1
export function notify(message: string, options?: NotifyOptions) {
    const id = nextId++
    const item: NotifyItem = {
        id,
        message,
        color: options?.color ?? 'success',
        timeout: options?.timeout ?? 3000,
        variant: options?.variant ?? 'outlined'
    }
    notifyQueue.push(item)

    // 自动移除
    setTimeout(() => {
        const index = notifyQueue.findIndex(i => i.id === id)
        if (index !== -1) notifyQueue.splice(index, 1)
    }, item.timeout)
}
