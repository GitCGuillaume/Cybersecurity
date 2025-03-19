<?php
	try {
		$dtb = new PDO('mysql:host=mysql;dbname=information_schema', 'root', 'toor');
		echo "Connection to database MySql";
	}
	catch (PDOException $err) {
		die($sql);
	}
?>

<!DOCTYPE html>
<html>
<head>
	<title>Mysql infection</title>
</head
<body>
	<form action="post">
		<input />
		<input type="hidden" />
		<input type="submit" />
	</form>
	<a href="./mysql.php">mysql.php</a>
	<a href="./oracle.php">oracle.php</a>
</body>
</html>
