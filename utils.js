const readData = (data) => {
  const valueExpression = data.kind

  switch (valueExpression) {
    case 'Str':
      console.log(data.value)
      break;
    case 'Print':
      readData(data.value)
      break;
    default:
      console.log('n print')
      break;
  }
}

module.exports = { readData }