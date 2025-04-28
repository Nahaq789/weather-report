aws dynamodb \
  --endpoint-url http://localhost:8000 \
  create-table \
  --table-name Sensor \
  --attribute-definitions \
    AttributeName=area,AttributeType=S \
    AttributeName=time_stamp,AttributeType=S \
  --key-schema \
    AttributeName=area,KeyType=HASH \
    AttributeName=time_stamp,KeyType=RANGE \
  --billing-mode=PAY_PER_REQUEST