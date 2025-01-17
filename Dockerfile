FROM confluentinc/cp-kafka-connect:7.4.0

# Installer le connecteur S3
RUN confluent-hub install --no-prompt confluentinc/kafka-connect-s3:latest
