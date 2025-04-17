aws dynamodb \
  --endpoint-url http://localhost:8000 \
  create-table \
  --table-name Sensor \
  --attribute-definitions AttributeName=sensor_id,AttributeType=S \
  --key-schema AttributeName=sensor_id,KeyType=HASH \
  --provisioned-throughput ReadCapacityUnits=1,WriteCapacityUnits=5 \
  --billing-mode=PAY_PER_REQUEST
