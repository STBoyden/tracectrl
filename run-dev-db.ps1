$CONTAINER_NAME = "tracectrl-db"
$ADMINER_CONTAINER_NAME = "${CONTAINER_NAME}-adminer"
$DB_USER = "tracectrl"
$DB_NAME = "tracectrl"


if (Get-Command docker -ErrorAction SilentlyContinue) {} else {
	Write-Error "command 'docker' doesn't exist on system"
	exit 1
}

if (![System.IO.File]::Exists(".\.dev-ps-pass.ps1")) {
	if (Get-Command openssl -ErrorAction SilentlyContinue) {} else {
		Write-Error "command 'openssl' doesn't exist on system"
		exit 1
	}

	$password = (openssl rand -hex 32) | Out-String
	$password = $password.Trim()
	Write-Output "New DB password: $password - MAKE SURE TO *NOT* USE THIS IN PRODUCTION, IT IS PURELY FOR DEVELOPMENT PURPOSES"

	Write-Output "Set-Item -Path Env:DB_PASS -Value '$password'" | Out-File -FilePath ".\.dev-ps-pass.ps1"
}

. .\.dev-ps-pass.ps1
$DB_PASS = "$Env:DB_PASS"


if (!$DB_PASS) {
	Write-Error "empty DB_PASS, check .\.dev-ps-pass.ps1"
	exit 1
}

$CONNECTION_STRING = "postgresql://${DB_USER}:${DB_PASS}@localhost:5432/${DB_NAME}"

$answer = ""
if (docker container ls -a | findstr "${CONTAINER_NAME}") {
	$answer = Read-Host -Prompt "Docker container with name '${CONTAINER_NAME}' already exists, delete? (y, N)"
}

switch -Regex ($answer) {
	'^(Y|y)$' {
		docker container rm -f "${CONTAINER_NAME}"
		Write-Output "Removed ${CONTAINER_NAME} container"
		break;
	}
}

$answer = ""
if (docker container ls -a | findstr "${ADMINER_CONTAINER_NAME}") {
	$answer = Read-Host -Prompt "Docker container with name '${ADMINER_CONTAINER_NAME}' already exists, delete? (y, N)"
}

switch -Regex ($answer) {
	'^(Y|y)$' {
		docker container rm -f "${ADMINER_CONTAINER_NAME}"
		Write-Output "Removed ${ADMINER_CONTAINER_NAME} container"
		break;
	}
}


if (
	docker run --name $CONTAINER_NAME `
		-p "5432:5432" `
		-e POSTGRES_PASSWORD="${DB_PASS}" `
		-e POSTGRES_USER="${DB_USER}" `
		-e POSTGRES_DB="${DB_NAME}" `
		-d postgres
) {
	Write-Output "PostgresDB started..."
	Write-Output "Connection string: $CONNECTION_STRING"
	Write-Output "Stop container with 'docker stop $CONTAINER_NAME'"
}
else {
	Write-Error "Could not start Postgres database: please see above for errors, if present"
	exit 1
}

if (
	docker run --name $ADMINER_CONTAINER_NAME `
		-p "8080:8080" `
		--link "${CONTAINER_NAME}:db" `
		-e ADMINER_DEFAULT_SERVER=db `
		-d adminer
) {
	Write-Output "Adminer started..."
	Write-Output "Started on http://localhost:8080"
	Write-Output "Stop container with 'docker stop ${ADMINER_CONTAINER_NAME}'"
}
else {
	Write-Error "Could not start Adminer front-end: please see above for errors, if present"
	exit 1
}
