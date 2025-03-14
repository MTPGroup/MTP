import { PrismaClient } from '@prisma/client'
import fs from 'fs/promises'
import path from 'path'

const prisma = new PrismaClient()
const baseUrl = 'https://arona.hanasaki.tech/api/student'

// 学生默认prompt模板
function getDefaultPrompt(studentName) {
  return `你是来自蔚蓝档案的学生${studentName}，你应该表现得符合角色特性。
你需要保持角色的一致性，友好地与用户交流。
在对话中要展现出${studentName}的性格特点和说话方式。
请记住，你是在与用户私聊，要有亲切感。`
}

async function main() {
  try {
    console.log('开始初始化数据库...')

    // 获取学生数据
    console.log('正在获取学生数据...')
    const response = await fetch(baseUrl)
    const data = await response.json()

    // 尝试读取已有的学生prompt数据（如果存在）
    let studentPrompts = {}
    try {
      const promptsFile = await fs.readFile(
        path.join(process.cwd(), 'scripts', 'student-prompts.json'),
        'utf8'
      )
      studentPrompts = JSON.parse(promptsFile)
      console.log('已加载学生prompt预设')
    } catch (error) {
      console.log('未找到学生prompt预设文件，将使用默认预设')
    }

    // 导入每个学生并创建对应私聊
    for (const studentData of data.data) {
      const studentName = studentData.PersonalName
      const avatarUrl = `https://aronacdn.hanasaki.tech/images/student/icon/${studentData.Id}.webp`

      // 查找学生是否已存在
      const existingStudent = await prisma.student.findUnique({
        where: { name: studentName },
      })

      if (existingStudent) {
        // 如果学生已存在，更新avatars数组
        let avatars = []
        try {
          avatars = JSON.parse(existingStudent.avatars)
          if (!Array.isArray(avatars)) {
            avatars = [avatars] // 如果不是数组，转换为数组
          }
        } catch (e) {
          // 如果解析失败，说明可能不是JSON格式，将其作为单个元素
          avatars = [existingStudent.avatars]
        }

        // 检查是否已存在相同的avatar
        if (!avatars.includes(avatarUrl)) {
          avatars.push(avatarUrl)
        }

        // 更新学生记录
        await prisma.student.update({
          where: { id: existingStudent.id },
          data: {
            avatars: JSON.stringify(avatars),
            // 如果存在自定义prompt则不更新
          },
        })

        console.log(`已更新学生 ${studentName} 的头像数组`)
      } else {
        // 创建新学生记录，avatars作为JSON数组
        const student = await prisma.student.create({
          data: {
            name: studentName,
            avatars: JSON.stringify([avatarUrl]),
            prompt:
              studentPrompts[studentName] || getDefaultPrompt(studentName),
          },
        })

        // 为每个学生创建对话
        await prisma.conversation.create({
          data: {
            title: student.name,
            studentName: student.name,
          },
        })

        console.log(`已创建学生 ${student.name} 及对应对话`)
      }
    }

    console.log('数据库初始化完成！')
  } catch (error) {
    console.error('初始化数据库时出错:', error)
  } finally {
    await prisma.$disconnect()
  }
}

main()
