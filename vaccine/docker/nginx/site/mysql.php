<?php
try {
	#http://127.0.0.1/mysql.php?a=)'; drop table list;'
	$dtb = new PDO('mysql:host=mysql;dbname=test', 'root', 'toor');
	echo "Connected to database MySql";
	var_dump($dtb);
	$type = 'abc';
	if ($_GET['a']) {
		$type = $_GET['a'];
	}
	$query_str = "SELECT vals from list where vals = '$type'";
	var_dump($query_str);
	$res = $dtb->query($query_str);
	echo '<pre>', var_dump($res->fetchAll()) , '</pre>';
	$res->closeCursor();
	/*if ($_POST['txt']) {
		$post = $_POST['txt'];
		$insert = "INSERT INTO list(vals) VALUES ('$post')";
		echo 'res: <pre>', var_dump($insert) , '</pre>';
		$res = $dtb->prepare($insert);
		$res2 = $res->execute();
		unset($res);
	}
	if ($_POST['hid']) {
		echo 'post_hid: ', $_POST['hid'];
		$post = $_POST['hid'];
		$insert = "INSERT INTO list(vals) VALUES ('$post')";
		echo 'res: <pre>', var_dump($insert) , '</pre>';
		$res = $dtb->prepare($insert);
		$res2 = $res->execute();
		unset($res);
	}*/
	if ($_POST['txt']) {
		$post = $_POST['txt'];
		$query_str = "SELECT id, vals from list where vals = '$post'";
		var_dump($query_str);
		$res = $dtb->query($query_str);
		echo '<pre>', var_dump($res->fetchAll()) , '</pre>';
		$res->closeCursor();
	}
	if ($_GET['txt']) {
		$post = $_GET['txt'];
		$query_str = "SELECT id, vals from list where vals = '$post'";
		var_dump($query_str);
		$res = $dtb->query($query_str);
		echo '<pre>', var_dump($res->fetchAll()) , '</pre>';
		$res->closeCursor();
	}

	$query_str = "SELECT * from list";
	$res = $dtb->query($query_str);
	echo '<pre>', var_dump($res->fetchAll()) , '</pre>';
	$res->closeCursor();
	
}
catch (PDOException $err) {
	die($err);
}
?>

<!DOCTYPE html>
<html>
<head>
	<title>Mysql infection</title>
</head
<body>
	<h1>Post</h1>
	<form action="mysql.php" method="post">
		<input name="txt" type="text" />
		<input name="hid"  type="hidden" />
		<input type="submit" />
	</form>
	<h2>Get</h2>
	<form action="mysql.php" method="get">
		<input name="txt" type="text" />
		<input name="hid"  type="hidden" />
		<input type="submit" />
	</form>
	<a href="./mysql.php">mysql.php</a>
	<a href="./mysql.php?a=abc">mysql.php?a=abc</a>
	<a href="./sqlite.php">sqlite.php</a>
	<a href="./sqlite.php?a=abc">sqlite.php?a=abc</a>
</body>
</html>
