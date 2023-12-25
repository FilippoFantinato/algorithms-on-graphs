package graph

trait Graph

class DenseGraph(val _n: Int)
{
  private var matrix: Array[Array[Double]] = Array.ofDim(_m, _n)

  def n(): Long = n
  def m(): Long = m

  def vertices(): List[V]
}
