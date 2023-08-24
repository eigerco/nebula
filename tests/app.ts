import { exit } from 'process'
import { TestsRunner } from './testsrunner'

async function start() {
  const testsRunner = new TestsRunner()
  const res = await testsRunner.runTests()
  if (!res) {
    exit(-1)
  }
}

void start()
