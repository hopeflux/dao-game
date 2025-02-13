<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from "@tauri-apps/api/core";

const loading = ref(true)
const showSuccess = ref(false)
const isChecking = ref(false)
const userAnswer = ref('')
const answerStatus = ref("")
const chapterTitle = ref('')
const firstSentence = ref('')
const chapter = ref(-1)

async function fetchNewQuestion() {
    try {
        loading.value = true
        const response: any = await invoke('get_question')
        chapter.value = response["id"]
        chapterTitle.value = response.title
        firstSentence.value = response.firstSentence
        userAnswer.value = ""
        answerStatus.value = ""
    } finally {
        loading.value = false
    }
}

async function checkAnswer() {
    try {
        isChecking.value = true
        const response: any = await invoke('check_answer', {
            chapter: chapter.value,
            answer: userAnswer.value
        })

        if (response.correct) {
            showSuccess.value = true
            answerStatus.value = 'success'
            fetchNewQuestion()
        } else {
            userAnswer.value = response.content
            answerStatus.value = 'error'
        }
    } finally {
        isChecking.value = false
    }
}

function enter(e: KeyboardEvent) {
    if (!e.shiftKey) {
        e.stopPropagation();//Firefox阻止冒泡行为
        e.preventDefault(); //取消事件的默认动作*换行
        checkAnswer();
    }
}

onMounted(fetchNewQuestion)
</script>

<template>
    <main class="container">
        <div class="p-8 max-w-2xl mx-auto p-6 bg-white">
                <div class="flex justify-between items-center mb-4">
                    <n-text class="!mb-0 text-lg font-semibold text-teal-800">{{ chapterTitle }}</n-text>
                    <n-button @click="fetchNewQuestion"
                        class="transition-all duration-200 hover:bg-teal-500 hover:text-white">
                        换一题
                    </n-button>
                </div>

                <div v-if="loading" class="text-center">
                    <n-spin size="large" />
                </div>

                <div v-else>
                    <n-alert type="info" class="mb-6 bg-teal-50 border-teal-200">
                        <div class="font-semibold text-teal-800">章节首句：</div>
                        <div class="text-teal-700">{{ firstSentence }}</div>
                    </n-alert>

                    <div class="mb-4">
                        <n-input type="textarea" v-model:value="userAnswer" placeholder="输入答案" :autosize="{
                            minRows: 3,
                            maxRows: 3,
                        }" @keydown.enter="enter" class="w-full p-2 rounded-lg border border-teal-300"
                            :status="answerStatus" />
                    </div>

                    <div class="text-center">
                        <n-button type="primary" size="large" @click="checkAnswer" :disabled="isChecking"
                            class="w-full transition-all duration-200 hover:bg-teal-600">
                            提交答案
                        </n-button>
                    </div>
                </div>

            <n-modal v-model:show="showSuccess" preset="dialog" type="success">
                <template #header>
                    <div class="text-xl font-bold">🎉 回答正确！</div>
                </template>
                <div class="text-teal-700">太棒了！你已掌握此章精髓</div>
            </n-modal>

            <div v-if="answerStatus === 'error'" class="mt-4 text-center text-red-600 font-semibold">
                再试一次，仔细思考道德经的智慧！
            </div>
        </div>
    </main>
</template>

<style scoped>
/* 1. 增加动画效果 */
@keyframes bounce {
    0% {
        transform: translateY(0);
    }

    50% {
        transform: translateY(-10px);
    }

    100% {
        transform: translateY(0);
    }
}

/* 2. 给按钮加个点击反馈 */
button:hover {
    animation: bounce 0.3s ease-in-out;
}

/* 3. 自定义样式 */
.bg-teal-50 {
    background-color: #f0fdfa;
}

.border-teal-200 {
    border-color: #99f6e4;
}

.bg-gradient-to-r {
    background: linear-gradient(to right, #A7F3D0, #6EE7B7);
}
</style>