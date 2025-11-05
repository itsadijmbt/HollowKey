def COLOR_MAP = [
    SUCCESS : '#2EB886',
    FAILURE : '#A30200',
    UNSTABLE: '#DAA038',
    ABORTED : '#CCCCCC'
]

pipeline {
    agent { label 'built-in' }

    environment {
        AWS_REGION     = 'us-east-1'
        AWS_ACCOUNT_ID = '810244486469'
        ECR_REPO       = 'hollowkey'
        IMAGE_URI      = "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPO}"
        IMAGE_TAG      = "${env.BUILD_NUMBER}"
    }

    stages {

        stage('Checkout') {
            steps {
                checkout scm
            }
        }

        stage('Building the code') {
            steps {
                dir('backend/hollowkey_api') {
                    sh '''#!/usr/bin/env bash
set -e
source /var/lib/jenkins/.cargo/env
cargo build --release
'''
                }
            }
            post {
                success {
                    echo "++++++++++ BUILD SUCCESSFUL +++++++++++++++"
                }
                failure {
                    echo "====++++ Build failed ++++===="
                    slackSend(
                        channel: '#cicd-hollowkey',
                        color: COLOR_MAP['FAILURE'],
                        message: "Build Stage Failed ❌ Build ${env.JOB_NAME} #${env.BUILD_NUMBER} (<${env.BUILD_URL}|View>)"
                    )
                }
            }
        }

        stage('Testing the build code') {
            steps {
                dir('backend/hollowkey_api') {
                    sh '''#!/usr/bin/env bash
set -e
source /var/lib/jenkins/.cargo/env
cargo test -- --nocapture
'''
                }
            }
            post {
                success {
                    echo "====++++ Tests passed ++++===="
                }
                failure {
                    echo "====++++ Tests failed ++++===="
                    slackSend(
                        channel: '#cicd-hollowkey',
                        color: COLOR_MAP['FAILURE'],
                        message: "Testing Stage Failed ❌ Build ${env.JOB_NAME} #${env.BUILD_NUMBER} (<${env.BUILD_URL}|View>)"
                    )
                }
            }
        }

        stage('Building docker and pushing images') {
            steps {
                sh '''#!/usr/bin/env bash
set -e
aws --region ${AWS_REGION} ecr get-login-password | \
    docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com

docker build -t ${ECR_REPO}:${IMAGE_TAG} -f backend/hollowkey_api/Dockerfile backend/hollowkey_api
docker tag ${ECR_REPO}:${IMAGE_TAG} ${IMAGE_URI}:${IMAGE_TAG}
docker tag ${ECR_REPO}:${IMAGE_TAG} ${IMAGE_URI}:latest
docker push ${IMAGE_URI}:${IMAGE_TAG}
docker push ${IMAGE_URI}:latest
'''
            }
            post {
                success {
                    echo "++++++++++++++ IMAGES BUILT AND PUSHED TO ECR +++++++++++++"
                }
                failure {
                    echo "====++++ Image build/push failed ++++===="
                    slackSend(
                        channel: '#cicd-hollowkey',
                        color: COLOR_MAP['FAILURE'],
                        message: "ECR Push Failed ❌ Build ${env.JOB_NAME} #${env.BUILD_NUMBER} (<${env.BUILD_URL}|View>)"
                    )
                }
            }
        }
    }

    post {
        always {
            echo "Slack Notification"
            slackSend(
                channel: '#cicd-hollowkey',
                color: COLOR_MAP[currentBuild.currentResult],
                message:
