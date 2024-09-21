/**
 * @param {*} obj
 * @param {*} classFunction
 * @return {boolean}
 */
var checkIfInstanceOf = function (obj, classFunction) {
  if (!classFunction) {
    return false
  }

  if (typeof classFunction !== 'function') {
    return false
  }

  const ret =
    obj instanceof classFunction ||
    obj?.constructor instanceof classFunction ||
    obj?.constructor === classFunction

  if (ret) {
    return true
  }

  const parent = obj?.constructor?.__proto__?.prototype

  if (parent) {
    return checkIfInstanceOf(parent, classFunction)
  }

  return false
}

/**
 * checkIfInstanceOf(new Date(), Date); // true
 */
