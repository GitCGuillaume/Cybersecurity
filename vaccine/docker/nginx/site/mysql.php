<?php
try {
		$dtb = new PDO('mysql:host=mysql;dbname=information_schema', 'root', 'toor');
		echo "Connected to database MySql";
		var_dump($dtb);
		$type = 'TABLESPACE';
		if ($_GET['a']) {
			$type = $_GET['a'];
		}
		$query_str = "SELECT * from ENGINES";# where FILE_TYPE = '$type'";
		$res = $dtb->query($query_str);
		echo '<pre>', var_dump($res->fetch()) , '</pre>';
		if ($_POST['txt']) {
			echo 'post_txt: ', $_POST['txt'];
			$post = $_POST['txt'];
			$insert = "INSERT INTO ENGINES(ENGINE, SUPPORT, COMMENT, TRANSACTIONS, XA, SAVEPOINTS) VALUES (?,?,?,?,?,?)";
			$res = $dtb->prepare($insert);
			$res2 = $res->execute([$post, $post, $post, $post, $post, $post]);
			echo 'res: <pre>', var_dump($res) , '</pre>';
			
		}
		if ($_POST['hid']) {
			echo 'post_hid: ', $_POST['hid'];
		}
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
	<form action="mysql.php" method="post">
		<input name="txt" type="text" />
		<input name="hid"  type="hidden" />
		<input type="submit" />
	</form>
	<a href="./mysql.php">mysql.php</a>
	<a href="./mysql.php?a=TABLESPACE">mysql.php?a=TABLESPACE</a>
	<a href="./sqlite.php">sqlite.php</a>
	<a href="./sqlite.php?a=TABLESPACE">sqlite.php?a=TABLESPACE</a>
</body>
</html>
