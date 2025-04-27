aws dynamodb \
  --endpoint-url http://localhost:8000 \
  create-table \
  --table-name Sensor \
  --attribute-definitions \
    AttributeName=sensor_id,AttributeType=S \
    AttributeName=time_stamp,AttributeType=S \
    AttributeName=area,AttributeType=S \
  --key-schema \
    AttributeName=sensor_id,KeyType=HASH \
    AttributeName=time_stamp,KeyType=RANGE \
  --global-secondary-indexes \
    "[
      {
        \"IndexName\": \"AreaIndex\",
        \"KeySchema\": [
          {\"AttributeName\": \"area\", \"KeyType\": \"HASH\"}
        ],
        \"Projection\": {
          \"ProjectionType\": \"ALL\"
        }
      }
    ]" \
  --billing-mode=PAY_PER_REQUEST
