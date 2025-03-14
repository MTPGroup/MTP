import { CSVLoader } from '@langchain/community/document_loaders/fs/csv'
import { AlibabaTongyiEmbeddings } from '@langchain/community/embeddings/alibaba_tongyi'
import { HNSWLib } from '@langchain/community/vectorstores/hnswlib'
import { RecursiveCharacterTextSplitter } from '@langchain/textsplitters'
import * as fs from 'fs'

// 添加延时函数
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms))

// 加载CSV文件函数
async function loadCSVFiles(paths) {
  const allDocs = []
  for (const filePath of paths) {
    try {
      console.log(`加载文件: ${filePath}`)
      const loader = new CSVLoader(filePath)
      const docs = await loader.load()
      allDocs.push(...docs)
      console.log(`成功加载: ${filePath}, 文档数: ${docs.length}`)
    } catch (error) {
      console.error(`加载文件失败: ${filePath}`, error)
    }
  }
  return allDocs
}

// 主函数
async function createVectorDatabase() {
  const vectorStorePath = './src/assets/vector_storage'

  // 检查目录是否存在，不存在则创建
  if (!fs.existsSync(vectorStorePath)) {
    fs.mkdirSync(vectorStorePath, { recursive: true })
  }

  try {
    console.log('初始化嵌入模型...')
    const embeddings = new AlibabaTongyiEmbeddings({
      apiKey: '<your-api-key>',
      modelName: 'text-embedding-v2',
    })

    console.log('创建HNSWLib向量存储...')
    // 创建一个空的向量存储
    const vectorStore = await HNSWLib.fromDocuments([], embeddings, {
      space: 'cosine', // 使用余弦相似度
    })

    // 文件列表
    const filePaths = [
      './src/assets/datasets/currency.csv',
      './src/assets/datasets/enemies.csv',
      './src/assets/datasets/equipment.csv',
      './src/assets/datasets/furniture.csv',
      './src/assets/datasets/game_data_rag.csv',
      './src/assets/datasets/items.csv',
      './src/assets/datasets/localization.csv',
      './src/assets/datasets/students.csv',
      './src/assets/datasets/summons.csv',
      './src/assets/datasets/voice.csv',
    ]

    // 分批处理文件
    const batchSize = 2 // 每次处理两个文件
    const fileDelay = 5000 // 文件批次间延迟5秒

    for (let i = 0; i < filePaths.length; i += batchSize) {
      const batch = filePaths.slice(i, i + batchSize)
      console.log(
        `处理文件批次 ${Math.floor(i / batchSize) + 1}/${Math.ceil(
          filePaths.length / batchSize
        )}`
      )

      // 加载这批文件
      const docs = await loadCSVFiles(batch)
      if (docs.length === 0) continue

      console.log(`分割文档...`)
      const splitter = new RecursiveCharacterTextSplitter({
        chunkSize: 1000,
        chunkOverlap: 200,
      })

      const allSplits = await splitter.splitDocuments(docs)
      console.log(`创建了 ${allSplits.length} 个文本块`)

      // 分批处理文本块
      const chunkBatchSize = 20 // 每次处理20个文本块
      const chunkDelay = 0 // 文本块批次间延迟2秒

      for (let j = 0; j < allSplits.length; j += chunkBatchSize) {
        const chunkBatch = allSplits.slice(j, j + chunkBatchSize)
        console.log(
          `处理文本块批次 ${Math.floor(j / chunkBatchSize) + 1}/${Math.ceil(
            allSplits.length / chunkBatchSize
          )}`
        )

        // 添加文本块到向量存储
        try {
          await vectorStore.addDocuments(chunkBatch)
          console.log('批次添加成功')
        } catch (error) {
          console.error('添加文档失败，等待10秒后重试...', error)
          await delay(10000) // 出错后等待10秒
          try {
            await vectorStore.addDocuments(chunkBatch)
            console.log('重试成功')
          } catch (retryError) {
            console.error('重试失败，跳过此批次', retryError)
          }
        }

        // 不是最后一批则等待
        if (j + chunkBatchSize < allSplits.length) {
          console.log(`等待 ${chunkDelay / 1000} 秒...`)
          await delay(chunkDelay)
        }
      }

      // 每处理完一批文件就保存一次向量库
      console.log(`保存向量库进度...`)
      await vectorStore.save(vectorStorePath)

      // 不是最后一批文件则等待
      if (i + batchSize < filePaths.length) {
        console.log(`等待 ${fileDelay / 1000} 秒后处理下一批文件...`)
        await delay(fileDelay)
      }
    }

    console.log('所有文件处理完成，保存最终向量库')
    await vectorStore.save(vectorStorePath)
    console.log('向量库创建成功，保存在路径:', vectorStorePath)
  } catch (error) {
    console.error('创建向量库失败:', error)
  }
}

// 执行主函数
createVectorDatabase()
  .then(() => console.log('脚本执行完成'))
  .catch((err) => console.error('脚本执行失败:', err))
