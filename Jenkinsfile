pipeline {
    agent any


    environment {
        AWS_REGION = 'us-east-1'
        AWS_ACCOUNT_ID = '810244486469'
        ECR_REPO = 'hollowkey'
        IMAGE_URI = "${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com/${ECR_REPO}"
        IMAGE_TAG = "${env.BUILD_NUMBER}"
    }

    stages {

        stage("Fetching Code from git :)") {
            steps {
                checkout scm
            }
            post {
                success {
                    echo "++++++++++ FETCHING SUCCESSFULL +++++++++++++++"
                }
                failure {
                    echo "!!!!!!!!!!!!!! FETCHING FAILED !!!!!!!!!!!!!!!!"
                }
            }
        }

        stage("Building the code :") {
            steps {
                sh '''
                source $HOME/.cargo/env
                cargo build --release
                '''
            }
            post {
                success {
                    echo "++++++++++ BUILD SUCCESSFULL +++++++++++++++"
                }
                failure {
                    echo "====++++Testing the build code execution failed++++===="
                    slackSend(
                        channel: '#cicd-hollowkey',
                        color: COLOR_MAP[currentBuild.currentResult],
                        message: "Build Stage Failed ❌ Build ${env.JOB_NAME} #${env.BUILD_NUMBER} failed (<${env.BUILD_URL}|View>)"
                    )
                }
            }
        }

        stage("Testing the build code") {
            steps {
                sh '''
                source $HOME/.cargo/env
                cargo test --
                '''
            }
            post {
                success {
                    echo "====++++Testing the build code executed successfully++++===="
                }
                failure {
                    echo "====++++Testing the build code execution failed++++===="
                    slackSend(
                        channel: '#cicd-hollowkey',
                        color: COLOR_MAP[currentBuild.currentResult],
                        message: "Testing Stage Failed ❌ Build ${env.JOB_NAME} #${env.BUILD_NUMBER} failed (<${env.BUILD_URL}|View>)"
                    )
                }
            }
        }

        stage("Building docker and pushing images") {
            steps {
                sh '''
                aws --region ${AWS_REGION} ecr get-login-password | \
                docker login --username AWS --password-stdin ${AWS_ACCOUNT_ID}.dkr.ecr.${AWS_REGION}.amazonaws.com

                docker build -t ${ECR_REPO}:${IMAGE_TAG} .
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
                    echo "====++++ image build and push failed++++===="
                    slackSend(
                        channel: '#cicd-hollowkey',
                        color: COLOR_MAP[currentBuild.currentResult],
                        message: "IMAGE & ECR Stage Failed ❌ Build ${env.JOB_NAME} #${env.BUILD_NUMBER} failed (<${env.BUILD_URL}|View>)"
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
                message: "SUCCESS !!!"
            )
        }
    }
}
