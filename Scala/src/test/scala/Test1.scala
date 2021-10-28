import org.junit.Test
import org.junit.Assert.*
import Instruction.*

class TestInst:
  @Test def parseInst =
    assertEquals(parse('+'), Some(Add(1)))
    assertEquals(parse('-'), Some(Add(-1)))
    assertEquals(parse('>'), Some(Move(1)))
    assertEquals(parse('<'), Some(Move(-1)))
    assertEquals(parse('['), Some(Loop(0)))
    assertEquals(parse(']'), Some(End(0)))
    assertEquals(parse('.'), Some(Print))
    assertEquals(parse(','), Some(Read))
    assertEquals(parse('a'), None)

  @Test def parseProg =
    val expect = List(Add(1), Move(1), Move(-1), Add(-1))
    val res = parseString("+>foo<-")
    assertEquals(expect, res)

  @Test def mapLoops =
    val expect = List(Add(1), Loop(3), Add(1), End(1))
    val res = parseString("+[+]")
    assertEquals(expect, res)

class TestOptimizers:
  @Test def groupInsts =
    val expect = List(Add(5), Move(3), Print, Loop(5), Add(-3), End(3), Move(-2), Read)
    val res = Grouper.optimize(parseString("+++++>>>.[---]<<."))

class TestRunner:
  val expected = "foo"
  val program = parseString("--[----->+<]>.+++++++++..")

  @Test def interpreter =
    var runner = Interpreter()
    runner.prepareCode(program)
    val res = runner.runProgram()
    assertEquals(expected, res)
  
  @Test def interpreterOpt = 
    var runner = Interpreter()
    val code = Grouper.optimize(program)
    runner.prepareCode(code)
    val res = runner.runProgram()
    assertEquals(expected, res)
