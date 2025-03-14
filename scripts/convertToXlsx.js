import * as fs from 'fs'
import * as path from 'path'
import * as xlsx from 'xlsx'
import { parse }  from 'csv-parse/sync'

// 源目录和目标目录
const sourceDir =  './src/assets/datasets'
const targetDir =  './src/assets/datasets_xlsx'

// 确保目标目录存在
if (!fs.existsSync(targetDir)) {
  fs.mkdirSync(targetDir, { recursive: true })
}

// 读取源目录中的所有CSV文件
const csvFiles = fs.readdirSync(sourceDir)
  .filter(file => file.endsWith('.csv'))
  .map(file => path.join(sourceDir, file))

// 转换每个CSV文件
console.log('开始转换CSV文件到XLSX...')
csvFiles.forEach(csvFile => {
  try {
    const fileName = path.basename(csvFile, '.csv')
    const targetFile = path.join(targetDir, `${fileName}.xlsx`)
    
    // 读取CSV内容
    console.log(`处理: ${fileName}.csv`)
    const csvContent = fs.readFileSync(csvFile, 'utf8')
    
    // 解析CSV
    const records = parse(csvContent, {
      columns: true,
      skip_empty_lines: true
    })
    
    // 创建工作簿和工作表
    const workbook = xlsx.utils.book_new()
    const worksheet = xlsx.utils.json_to_sheet(records)
    
    // 将工作表添加到工作簿
    xlsx.utils.book_append_sheet(workbook, worksheet, fileName)
    
    // 写入XLSX文件
    xlsx.writeFile(workbook, targetFile)
    
    console.log(`成功转换: ${fileName}.csv -> ${fileName}.xlsx`)
  } catch (error) {
    console.error(`转换文件失败: ${csvFile}`, error)
  }
})

console.log('转换完成!')
