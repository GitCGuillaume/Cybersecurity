<?php
	try {
		$dtb = new PDO('sqlite:/var/www/site/main.db');
		echo "Connected to database Sqlite";
		var_dump($dtb);
	}
	catch (PDOException $err) {
		die($sql);
	}
?>

<!DOCTYPE html>
<html>
<head>
	<title>Sqlite infection</title>
</head
<body>
	<form action="post">
		<input />
		<input type="hidden" />
		<input type="submit" />
	</form>
	<a href="./mysql.php">mysql.php</a>
	<a href="./sqlite.php">sqlite.php</a>
</body>
</html>
